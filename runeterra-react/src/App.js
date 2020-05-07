import React from 'react';
import './style/App.css';
import Login from './pages/Login';
import Main from './pages/main';

class App extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      authToken: null,
    };
  }

  handleAuthToken = (tokenValue) => {
    this.setState({authToken: tokenValue});
  }

  render() {
    return (
      <div className="App">
        {this.state.authToken != null && 
          <Main token={this.state.authToken} />
        }
        {this.state.authToken == null &&
          <Login onGetAuthToken={this.handleAuthToken} />
        }
      </div>
    );
  }
}

export default App;
