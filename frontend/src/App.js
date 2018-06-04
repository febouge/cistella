import React, { Component } from 'react';
import darkBaseTheme from 'material-ui/styles/baseThemes/darkBaseTheme';
import MuiThemeProvider from 'material-ui/styles/MuiThemeProvider';
import getMuiTheme from 'material-ui/styles/getMuiTheme';

import './App.css';
import Header from './header/Header.js';
import Footer from './footer/Footer.js';
import Content from './content/Content.js';

class App extends Component {
  render() {
    return (
      <MuiThemeProvider muiTheme={getMuiTheme(darkBaseTheme)}>
        <div className="App">
          <Header />
          <Content />
          <Footer />
        </div>
      </MuiThemeProvider>
    );
  }
}

export default App;
