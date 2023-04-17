import { useNavigate } from "react-router-dom";
import { useSession } from "../main";

export default function LoginPage() {
  const session = useSession();
  let navigate = useNavigate();

  return (
    <div className="page login-page">
      <form
        className="login-page__form"
        onSubmit={(e) => {
          e.preventDefault();
          session.login("artur", "Artur1", () => {
            navigate("/");
          });
        }}
      >
        <label htmlFor="login">Name:</label>
        <input type="text" name="login" id="login" />
        <label htmlFor="password">Password:</label>
        <input type="password" name="password" id="password" />
        <button className="btn">Login</button>
      </form>
      <div className="login-page__sign-up-prompt">
        <span>Don't have an account? </span>
        <a href="/sign-up">Sign up here</a>
      </div>
    </div>
  );
}
