import Link from "next/link";

type Props = {
  title: string;
  author: string;
  date: string;
  content: string;
  id: number;
}

export default function PostCard({ title, author, date, content, id }: Props){
  return (
    <Link href={`/post/${id}`} className="block">
      <div className="bg-white rounded-lg shadow p-4 mb-4">
        <h2 className="text-2xl mb-2">{title}</h2>
        <p className="text-gray-600">Author: {author}</p>
        <p className="text-gray-600">Date: {date}</p>
        <p className="mt-4">{content}</p>
      </div>
    </Link>
  );
};
