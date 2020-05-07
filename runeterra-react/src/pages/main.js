import React from 'react';

class Main extends React.Component {
    constructor(props) {
        super(props);
        this.state = {};
      }
      render() {
        return (
          <div className="MainComponent">
            <p>Omg I am authenticated</p>
            <p>Also my token is - {this.props.token}</p>
          </div>
        );
      }
}

export default Main
