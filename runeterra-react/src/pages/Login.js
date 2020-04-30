import React from 'react';

class Login extends React.Component {
    constructor(props) {
        super(props);
        this.state = {};
      }
      handleAuth = () => {
        var myToken = 5;
        this.props.onGetAuthToken(myToken);
      }
      render() {
        return (
          <div className="LoginComponent">
            <p>Login Foo</p>
            <button onClick={this.handleAuth}/>
          </div>
        );
      }
}

export default Login
