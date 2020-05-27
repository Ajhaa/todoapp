import Link from 'next/link'
import { useRouter } from 'next/router'

async function deleteTodo(id) {
  return fetch('http://localhost:3030/todos/' + id, {
    method: 'DELETE',
  })
}

export default function Home({ todos }) {
  const router = useRouter()

  const del = async (id) => {
    await deleteTodo(id)
    router.reload()
  }
  return (
    <div className="container">
      <Link href="/new"><a>new todo</a></Link>
      {todos.map(todo => <div key={todo.id}>{todo.content} <button onClick={() => del(todo.id)}>delete</button></div>)}
    </div>
  )
}

export async function getServerSideProps() {
  const res = await fetch('http://localhost:3030/todos')
  const todos = await res.json()

  return { props: { todos }}
}
