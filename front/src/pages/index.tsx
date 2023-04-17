import { Link } from "react-router-dom";
import { useSession } from "../main";

export default function IndexPage() {
  return (
    <div className="page">
      <h1>{"null"}</h1>
      <BottomBar />
    </div>
  );
}

const BottomBar = () => {
  let { username, isLoggedIn, logout } = useSession();

  return (
    <div className="bottom-bar">
      {isLoggedIn ? (
        <>
          <Link to={`/user/${username}`}>{username}</Link>
          <button className="btn" onClick={logout}>
            Logout
          </button>
        </>
      ) : (
        <Link to="/login">
          <button className="btn">Login</button>
        </Link>
      )}
    </div>
  );
};
