import React, { Component } from 'react';
import TextField from 'material-ui/TextField';
import './ShoppingListItem.css';


class ShoppingListItem extends Component {

  constructor(props) {
    super(props);
    this.state = {
      name: props.itemName,
      quantity: props.itemQuantity
    };
  }

  render() {
    return (
        <div className="ShoppingListItem">
          <TextField className="ShoppingListItem-TextField" type="text" floatingLabelText="Name" name="itemName" defaultValue={this.state.name} />
          <TextField className="ShoppingListItem-TextField" name="itemQuantity"  floatingLabelText="Quantity" defaultValue={this.state.quantity} />
        </div>
    );
  }
}

export default ShoppingListItem;
