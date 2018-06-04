import React, { Component } from 'react';
import './Content.css';
import ShoppingList from './shoppinglist/ShoppingList';
import SideBar from './sidebar/Sidebar';

class Content extends Component {

  constructor(props) {
    super(props);
    this.state = {
      activeShoppingListId: null
    };
  }

  showShoppingList(shoppingListId) {
    this.setState({
      activeShoppingListId: shoppingListId
    });
  }

  render() {
    return (
      <div className="Content">
        <SideBar elementsClickFunction={this.showShoppingList.bind(this)}/>
        <ShoppingList activeShoppingListId={this.state.activeShoppingListId} />
      </div>
    );
  }
}

export default Content;
