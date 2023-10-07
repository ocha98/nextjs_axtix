import Link from "next/link"

const GITHUB_OAUTH_CLIENT_ID = process.env.NEXT_PUBLIC_GITHUB_OAUTH_CLIENT_ID;

export default function Navbar() {
  return (
    <nav className="bg-gray-800 p-4">
      <div className="flex items-center justify-between">
        <div className="flex space-x-4">
          <div className="text-white text-lg">
            <Link href="/" className="text-white">Ocha</Link>
          </div>
          <div className="space-x-4">
            <Link href="/posts" className="text-white">Posts</Link>
            <Link href="#" className="text-white">About</Link>
            <Link href="#" className="text-white">Contact</Link>
          </div>
        </div>
        <div>
          <a href={"https://github.com/login/oauth/authorize?client_id=" + GITHUB_OAUTH_CLIENT_ID} className="text-white">Login</a>
        </div>
      </div>
    </nav>
  )
}