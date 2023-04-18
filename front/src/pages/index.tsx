import { Link } from "react-router-dom";
import { useSession } from "../main";
import { useEffect, useState } from "react";
import axios from "axios";

axios.defaults.baseURL = "http://127.0.0.1:8080/";

export default function IndexPage() {
  const { posts, hasNext, fetchMore } = usePostsHook();
  console.log(posts);

  return (
    <div className="page">
      <h1>{"null"}</h1>
      <BottomBar />
      <button
        onClick={() => {
          fetchMore();
        }}
      >
        dasdsa
      </button>
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
interface Post {
  id: String;
  username: String;
  content: String;
}

const usePostsHook = () => {
  const [posts, setPosts] = useState<Post[]>([]);
  const [page, setPage] = useState(0);
  const [hasNext, setHasNext] = useState(true);

  const fetchMore = () => {
    if (hasNext) {
      axios
        .put("/post", {
          page: page,
        })
        .then((res) => {
          setPage((prev) => prev + 1);
          setPosts((prev) => [...prev, ...res.data.posts]);
          setHasNext(res.data.has_next);
        });
    }
  };

  return { posts, hasNext, fetchMore };
};
