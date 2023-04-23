import { Link } from "react-router-dom";
import { useSession } from "../main";
import { useState } from "react";
import axios from "axios";
import InfiniteScrollTrigger from "../components/InfiniteScrollTrigger";
import { useForm } from "react-hook-form";

export default function IndexPage() {
  const { posts, hasNext, isFetching, fetchMore, refetch } = usePostsHook();

  return (
    <div className="page index-page">
      <PostInput onSubmit={refetch} />
      <BottomBar />
      <PostsList posts={posts} />
      {!isFetching && hasNext && (
        <InfiniteScrollTrigger onScreenEnter={fetchMore} />
      )}
    </div>
  );
}

const PostInput = ({ onSubmit }: { onSubmit: () => void }) => {
  const {
    register,
    handleSubmit,
    reset,
    formState: { errors },
  } = useForm();

  return (
    <form
      className="post-input"
      onSubmit={handleSubmit((data) => {
        axios
          .post(
            "/post",
            {
              content: data.content,
            },
            {
              headers: {
                "Access-Control-Allow-Origin": "http://127.0.0.1:8080",
              },
            }
          )
          .then((res) => {
            onSubmit();
          });
        reset();
      })}
    >
      <textarea {...register("content")} />
      <button className="btn">Submit</button>
    </form>
  );
};

const PostsList = ({ posts }: { posts: Post[] }) => {
  return (
    <ul className="posts-list">
      {posts.map(({ username, id, content }) => (
        <li className="posts-list__item" key={id}>
          <p className="posts-list__item__username">{username}</p>
          <p className="posts-list__item__content">{content}</p>
        </li>
      ))}
    </ul>
  );
};

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

  const refetch = () => {
    setPage(0);
    setHasNext(true);
  };

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

  return { posts, hasNext, isFetching, fetchMore, refetch };
};
