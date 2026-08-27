// Frontend components/StatusBar.jsx

import React from 'react'
import './StatusBar.css'

function StatusBar({ language, theme, onThemeChange, onSave }) {
  const currentTime = new Date().toLocaleTimeString()
  const [time, setTime] = React.useState(currentTime)

  React.useEffect(() => {
    const timer = setInterval(() => {
      setTime(new Date().toLocaleTimeString())
    }, 1000)
    return () => clearInterval(timer)
  }, [])

  return (
    <div className="status-bar">
      <div className="status-left">
        <span className="status-item">
          Language: <strong>{language.toUpperCase()}</strong>
        </span>
        <span className="status-item">UTF-8</span>
        <span className="status-item">CRLF</span>
      </div>

      <div className="status-center">
        <button 
          className="btn-theme"
          onClick={() => onThemeChange(theme === 'vs-dark' ? 'vs-light' : 'vs-dark')}
        >
          🌙 {theme === 'vs-dark' ? 'Dark' : 'Light'}
        </button>
      </div>

      <div className="status-right">
        <button className="btn-save" onClick={onSave}>💾 Save</button>
        <span className="status-item">{time}</span>
      </div>
    </div>
  )
}

export default StatusBar
