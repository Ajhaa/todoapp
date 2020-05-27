import { useState } from 'react'
import { useRouter } from 'next/router'

async function postTodo(content) {
  let response = await fetch('http://localhost:3030/todos', {
    body: JSON.stringify({ content }),
    headers: { 'Content-Type': 'application/json' },
    method: 'POST'
  })

  return response
}

export default function NewTodo() {
  const router = useRouter()
  const [content, setContent] = useState('')

  const handleSubmit = async (event) => {
    event.preventDefault()

    const res = await postTodo(content)
    if (res.ok) {
      router.push('/')
    } else {
      console.log(res)
    }
  }

  return (
    <div>
      <form onSubmit={handleSubmit}>
        <div>
          <label>content</label>
          <input 
            onChange={e => setContent(e.target.value)}
            value={content} 
            type="text" 
          />
        </div>
        <div>
          <button type="submit">submit</button>
        </div>
      </form>
    </div>
  )
}