import React, { createContext, useContext, useEffect, useState } from "react";
import ReactDOM from "react-dom/client";
import {
  createBrowserRouter,
  RouterProvider,
  useNavigate,
} from "react-router-dom";
import "./styles/index.scss";
import IndexPage from "./pages";
import LoginPage from "./pages/login";
import SignUpPage from "./pages/sign-up";
import { useCookies } from "react-cookie";
import axios from "axios";

axios.defaults.baseURL = "http://127.0.0.1:8080/";

const router = createBrowserRouter([
  {
    path: "/",
    element: <IndexPage />,
  },
  {
    path: "/login",
    element: <LoginPage />,
  },
  {
    path: "/sign-up",
    element: <SignUpPage />,
  },
]);

interface SessionDataType {
  username: string | null;
  login: (username: string, password: string, onSucces?: () => void) => void;
  logout: () => void;
}

const SessionContext = createContext<SessionDataType>({
  username: null,
  login: () => {},
  logout: () => {},
});

const SessionProvider = ({ children }: { children: React.ReactNode }) => {
  const [cookies, setCookie, removeCookie] = useCookies(["session-token"]);
  const [sessionData, setSessionData] = useState<SessionDataType>({
    username: "dsadsad",
    login: (username, password, onSucces) => {
      axios
        .post("/user/login", {
          username,
          password,
        })
        .then(function (response) {
          if (onSucces) {
            onSucces();
          }

          setCookie("session-token", response.data.session_token);
          setSessionData({ ...sessionData, username: response.data.username });
        })
        .catch(function (error) {
          alert("Wrong credentials");
        });
    },
    logout: () => {
      axios
        .post("/user/logout", {
          session_token: cookies["session-token"],
        })
        .then(function () {
          removeCookie("session-token");
          setSessionData({ ...sessionData, username: null });
        })
        .catch(function () {
          alert("There was an issue during logging you out");
        });
    },
  });

  // if have token -> fetch session data
  useEffect(() => {}, []);

  return (
    <SessionContext.Provider value={sessionData}>
      <>{children}</>
    </SessionContext.Provider>
  );
};

export const useSession = () => {
  return useContext(SessionContext);
};

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <SessionProvider>
      <RouterProvider router={router} />
    </SessionProvider>
  </React.StrictMode>
);
