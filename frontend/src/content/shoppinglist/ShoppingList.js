import React, { Component } from 'react';
import AppBar from 'material-ui/AppBar';
import ShoppingListItem from './ShoppingListItem';
import './ShoppingList.css';

class ShoppingList extends Component {

  constructor(props) {
    super(props);
    this.state = {
      id: props.activeShoppingListId,
      items: []
    };
  }

  componentDidUpdate(prevProps, prevState) {
    if (prevProps.activeShoppingListId === this.props.activeShoppingListId){
      return false;
    }
    fetch('http://localhost:8000/shoppinglist/'.concat(this.props.activeShoppingListId))
    .then(response => response.json())
    .then(shoppingList => this.setState({ id: shoppingList.date, items: shoppingList.items }))
  }

  render() {
    return (
        <div className="ShoppingList">
          <AppBar title={this.state.id} iconClassNameRight="muidocs-icon-navigation-expand-more" />
          <ul>
            {
              Object.keys(this.state.items).map(key => <ShoppingListItem  key={key} itemName={key} itemQuantity={this.state.items[key]}/>)
            }
          </ul>
        </div>
    );
  }
}

export default ShoppingList;
