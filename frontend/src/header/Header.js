import React, { Component } from 'react';
import AppBar from 'material-ui/AppBar';
import './Header.css';

class Header extends Component {
  render() {
    return (
      <header className="App-header">
          <AppBar title="Cistella" iconClassNameRight="muidocs-icon-navigation-expand-more" />
      </header>
    );
  }
}

export default Header;
