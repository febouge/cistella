import React, { Component } from 'react';
import './Sidebar.css';
import SidebarEntry from './SidebarEntry.js';

class Sidebar extends Component {

  constructor(props) {
    super(props);
    this.state = { shoppingLists: [] };
  }

  componentDidMount() {
    fetch('http://localhost:8000/shoppinglists')
    .then(response => response.json())
    .then(shoppingLists => this.setState({ shoppingLists: shoppingLists }))
  }

  render() {
    if (this.state.shoppingLists.length > 0) {
      return (
        <div className="Sidebar">
          <ul>
            {
              this.state.shoppingLists.map((shoppingListDate, index) => <SidebarEntry key={ index } name={ shoppingListDate } onClick={this.props.elementsClickFunction}/>)
            }
          </ul>
        </div>
      );
    } else {
      return <div className="Sidebar">Loading...</div>;
    }
  }
}

export default Sidebar;
