import { Link } from "react-router-dom";
import { useSession } from "../main";
import { useEffect, useState } from "react";
import axios from "axios";
import InfiniteScrollTrigger from "../components/InfiniteScrollTrigger";

axios.defaults.baseURL = "http://127.0.0.1:8080/";

export default function IndexPage() {
  const { posts, hasNext, isFetching, fetchMore } = usePostsHook();

  return (
    <div className="page">
      <h1>{"Recent posts"}</h1>
      <BottomBar />
      {posts.map(({ content, id }) => (
        <div key={id} style={{ height: "150px" }}>
          {content}
        </div>
      ))}
      {!isFetching && hasNext && (
        <InfiniteScrollTrigger onScreenEnter={fetchMore} />
      )}
      <h2>dasdas</h2>
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
  id: string;
  username: string;
  content: string;
}

const usePostsHook = () => {
  const [posts, setPosts] = useState<Post[]>([]);
  const [page, setPage] = useState(0);
  const [hasNext, setHasNext] = useState(true);
  const [isFetching, setIsFetching] = useState(false);

  const fetchMore = () => {
    if (hasNext && !isFetching) {
      setIsFetching(true);
      axios
        .put("/post", {
          page: page,
        })
        .then((res) => {
          setPage((prev) => prev + 1);
          setPosts((prev) => [...prev, ...res.data.posts]);
          setHasNext(res.data.has_next);
        })
        .finally(() => {
          setIsFetching(false);
        });
    }
  };

  return { posts, hasNext, isFetching, fetchMore };
};
