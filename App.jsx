// Frontend App.jsx - Main application component

import React, { useState, useEffect } from 'react'
import Editor from '@monaco-editor/react'
import Sidebar from './components/Sidebar'
import Terminal from './components/Terminal'
import StatusBar from './components/StatusBar'
import './App.css'

function App() {
  const [theme, setTheme] = useState('vs-dark')
  const [language, setLanguage] = useState('javascript')
  const [code, setCode] = useState('// Welcome to Code Maestro\nconsole.log("Hello, World!")')
  const [files, setFiles] = useState([])
  const [currentFile, setCurrentFile] = useState(null)
  const [suggestions, setSuggestions] = useState([])

  useEffect(() => {
    // Load initial files
    const initialFiles = [
      { id: 1, name: 'index.js', language: 'javascript', content: '' },
      { id: 2, name: 'style.css', language: 'css', content: '' },
      { id: 3, name: 'main.rs', language: 'rust', content: '' },
    ]
    setFiles(initialFiles)
  }, [])

  const handleEditorChange = (value) => {
    setCode(value)
    // Trigger AI suggestions
    fetchSuggestions(value, language)
  }

  const fetchSuggestions = async (code, lang) => {
    try {
      // This would call the backend API
      // const response = await fetch('/api/suggestions', {
      //   method: 'POST',
      //   body: JSON.stringify({ code, language: lang })
      // })
      // const data = await response.json()
      // setSuggestions(data.suggestions)
    } catch (error) {
      console.error('Error fetching suggestions:', error)
    }
  }

  const handleFileSelect = (file) => {
    setCurrentFile(file)
    setLanguage(file.language)
  }

  const handleNewFile = () => {
    const newFile = {
      id: files.length + 1,
      name: `untitled-${files.length + 1}.js`,
      language: 'javascript',
      content: '',
    }
    setFiles([...files, newFile])
    setCurrentFile(newFile)
  }

  const handleSave = () => {
    console.log('Saving file...', currentFile)
    // TODO: Implement file saving
  }

  return (
    <div className="app-container">
      <div className="editor-layout">
        <Sidebar 
          files={files}
          currentFile={currentFile}
          onFileSelect={handleFileSelect}
          onNewFile={handleNewFile}
          suggestions={suggestions}
        />
        
        <div className="editor-main">
          <div className="editor-tabs">
            {files.map(file => (
              <div 
                key={file.id}
                className={`tab ${currentFile?.id === file.id ? 'active' : ''}`}
                onClick={() => handleFileSelect(file)}
              >
                {file.name}
              </div>
            ))}
          </div>

          <div className="editor-content">
            <Editor
              height="100%"
              language={language}
              value={code}
              onChange={handleEditorChange}
              theme={theme}
              options={{
                minimap: { enabled: true },
                fontSize: 14,
                fontFamily: 'Fira Code, monospace',
                wordWrap: 'on',
                autoIndent: 'full',
              }}
            />
          </div>
        </div>
      </div>

      <div className="bottom-panel">
        <Terminal />
      </div>

      <StatusBar 
        language={language}
        theme={theme}
        onThemeChange={setTheme}
        onSave={handleSave}
      />
    </div>
  )
}

export default App
