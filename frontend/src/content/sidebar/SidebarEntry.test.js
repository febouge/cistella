import React from 'react';
import ReactDOM from 'react-dom';
import SidebarEntry from './SidebarEntry';

it('SidebarEntry renders without crashing', () => {
  const div = document.createElement('div');
  ReactDOM.render(<SidebarEntry />, div);
  ReactDOM.unmountComponentAtNode(div);
});
