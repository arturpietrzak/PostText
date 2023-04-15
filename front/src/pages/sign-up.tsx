export default function SignUpPage() {
  return (
    <div className="page sign-up-page">
      <form
        className="sign-up-page__form"
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
      <div className="sign-up-page__login-prompt">
        <span>Already have an account? </span>
        <a href="/login">Login here</a>
      </div>
    </div>
  );
}
