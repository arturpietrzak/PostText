import { useSession } from "../main";

export default function IndexPage() {
  let { username } = useSession();
  return <h1>{username ?? "null"}</h1>;
}
