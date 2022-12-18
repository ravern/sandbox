import React, { useState, useEffect } from 'react';
import ReactDOM from 'react-dom';

import { IntlProvider, defineMessages, FormattedMessage } from "react-intl";

const messages = defineMessages({
  en: require('../i18n/en/strings.json'),
  zh: require('../i18n/zh/strings.json'),
});

function App() {
  const [language, setLanguage] = useState('en');

  return <IntlProvider locale={language} messages={messages[language]}>
    <select onChange={e => setLanguage(e.target.value)} value={language}>
      <option>en</option>
      <option>zh</option>
    </select>
    <h1><FormattedMessage id="helloWorld" /></h1>
    <h2><FormattedMessage id="greeting" values={{ name: "Ravern" }} /></h2>
  </IntlProvider>
}

const root = document.getElementById('root');
ReactDOM.render(<App />, root)
