import React, { useState, useEffect } from 'react';
import ReactDOM from 'react-dom';

function App() {
  const [language, setLanguage] = useState('es');
  
  useEffect(() => {
    // TODO: Change the language
  }, [language]);

  return <LangProvider client={client}>
    <select onChange={e => setLanguage(e.target.value)} value={language}>
      <option>en</option>
      <option>zh</option>
    </select>
    <h1>Hello, world!</h1>
    <h2>I am Ravern.</h2>
  </LangProvider>
}

const root = document.getElementById('root');
ReactDOM.render(<App />, root)
