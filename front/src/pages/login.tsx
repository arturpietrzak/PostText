export default function LoginPage() {
  return (
    <div className="page login-page">
      <form
        className="login-page__form"
        onSubmit={(e) => {
          e.preventDefault();
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
