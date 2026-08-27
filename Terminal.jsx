// Frontend components/Terminal.jsx

import React, { useState } from 'react'
import './Terminal.css'

function Terminal() {
  const [output, setOutput] = useState([
    '$ Code Maestro Terminal Ready',
    '> Type commands here...'
  ])
  const [input, setInput] = useState('')

  const handleCommand = (e) => {
    if (e.key === 'Enter') {
      const newOutput = [...output, `$ ${input}`]
      
      // Simple command processing
      if (input.toLowerCase() === 'clear') {
        setOutput([])
      } else if (input.toLowerCase().startsWith('echo ')) {
        newOutput.push(input.substring(5))
      } else if (input.toLowerCase() === 'help') {
        newOutput.push('Available commands:')
        newOutput.push('  clear - Clear terminal')
        newOutput.push('  echo <text> - Print text')
        newOutput.push('  help - Show this message')
      } else {
        newOutput.push(`Command not found: ${input}`)
      }
      
      setOutput(newOutput)
      setInput('')
    }
  }

  return (
    <div className="terminal">
      <div className="terminal-header">
        <span>Terminal</span>
        <button className="btn-close">×</button>
      </div>
      <div className="terminal-output">
        {output.map((line, index) => (
          <div key={index} className="terminal-line">{line}</div>
        ))}
      </div>
      <div className="terminal-input">
        <input
          type="text"
          value={input}
          onChange={(e) => setInput(e.target.value)}
          onKeyPress={handleCommand}
          placeholder="Enter command..."
          autoFocus
        />
      </div>
    </div>
  )
}

export default Terminal
