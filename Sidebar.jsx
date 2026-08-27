// Frontend components/Sidebar.jsx

import React from 'react'
import './Sidebar.css'

function Sidebar({ files, currentFile, onFileSelect, onNewFile, suggestions }) {
  return (
    <div className="sidebar">
      <div className="sidebar-header">
        <h2>Code Maestro</h2>
        <button className="btn-new-file" onClick={onNewFile} title="New File">
          +
        </button>
      </div>

      <div className="sidebar-section">
        <h3>Explorer</h3>
        <div className="file-tree">
          {files.map(file => (
            <div
              key={file.id}
              className={`file-item ${currentFile?.id === file.id ? 'active' : ''}`}
              onClick={() => onFileSelect(file)}
            >
              <span className="file-icon">📄</span>
              <span className="file-name">{file.name}</span>
            </div>
          ))}
        </div>
      </div>

      <div className="sidebar-section">
        <h3>Suggestions</h3>
        <div className="suggestions-list">
          {suggestions.length > 0 ? (
            suggestions.map((suggestion, index) => (
              <div key={index} className="suggestion-item">
                <p className="suggestion-text">{suggestion.code}</p>
                <small className="suggestion-confidence">
                  Confidence: {(suggestion.confidence * 100).toFixed(0)}%
                </small>
              </div>
            ))
          ) : (
            <p className="empty-message">No suggestions yet</p>
          )}
        </div>
      </div>

      <div className="sidebar-footer">
        <button className="btn-settings">⚙️ Settings</button>
      </div>
    </div>
  )
}

export default Sidebar
