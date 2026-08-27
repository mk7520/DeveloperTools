// Frontend utils/api.js - API client for backend communication

export const API_BASE = import.meta.env.VITE_API_URL || 'http://localhost:3000/api'

export const api = {
  // Code suggestions
  getSuggestions: async (code, language, context) => {
    const response = await fetch(`${API_BASE}/suggestions`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ code, language, context })
    })
    return response.json()
  },

  // Generate boilerplate
  generateBoilerplate: async (language, pattern) => {
    const response = await fetch(`${API_BASE}/generate`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ language, pattern })
    })
    return response.json()
  },

  // Check syntax
  checkSyntax: async (code, language) => {
    const response = await fetch(`${API_BASE}/syntax-check`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ code, language })
    })
    return response.json()
  },

  // Save file
  saveFile: async (filePath, content) => {
    const response = await fetch(`${API_BASE}/files/save`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ filePath, content })
    })
    return response.json()
  },

  // Load file
  loadFile: async (filePath) => {
    const response = await fetch(`${API_BASE}/files/${filePath}`)
    return response.json()
  },

  // Get snippets
  getSnippets: async (language) => {
    const response = await fetch(`${API_BASE}/snippets?language=${language}`)
    return response.json()
  },

  // Save snippet
  saveSnippet: async (name, code, language, category) => {
    const response = await fetch(`${API_BASE}/snippets`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ name, code, language, category })
    })
    return response.json()
  },

  // Get settings
  getSettings: async () => {
    const response = await fetch(`${API_BASE}/settings`)
    return response.json()
  },

  // Save settings
  saveSettings: async (settings) => {
    const response = await fetch(`${API_BASE}/settings`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(settings)
    })
    return response.json()
  }
}

export default api
