import React, { Component } from 'react';
import FlatButton from 'material-ui/FlatButton';
import './SidebarEntry.css';

class SidebarEntry extends Component {

  constructor(props) {
    super(props);
    this.onClick = this.onClick.bind(this);
  }

  onClick() {
    this.props.onClick(this.props.name);
  }

  render() {
    return (
      <FlatButton label={this.props.name} className="SidebarEntry" onClick={this.onClick} />
    );
  }
}

export default SidebarEntry;
