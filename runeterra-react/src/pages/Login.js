import React from 'react';
import { 
  Button,
  Form,
  FormGroup,
  Label,
  Input
} from 'reactstrap';
import '../style/login.css'
const axios = require('axios').default;

class Login extends React.Component {
    constructor(props) {
        super(props);
        this.state = {
          inputValue: "",
          passwordValue: ""
        };
      }
      handleAuth = () => {
        let user = this.state.inputValue;
        let pass = this.state.passwordValue;
        if(user!== "" && pass !== "") {
          var myToken = 5;
          axios.get(`/auth?username=${user}&password=${pass}`)
            .then(res => {
              console.log(res)
              this.props.onGetAuthToken(myToken);
            })
            .catch(e => {
              console.log(e)
            });
        }
      }
      updateInputValue(evt) {
        this.setState({
          inputValue: evt.target.value
        })
      }
      updatePasswordValue(evt) {
        this.setState({
          passwordValue: evt.target.value
        })
      }
      render() {
        return (
          <div className="LoginComponent">
            <Form>
              <FormGroup className="input-form-comp">
                <Label for="Email">Email: </Label>
                <Input type="email" name="email" id="EmailInput" onChange={e => this.updateInputValue(e)} placeholder="Enter Email" />
              </FormGroup>
              <FormGroup className="input-form-comp">
                <Label for="Password">Password: </Label>
                <Input type="password" name="password" id="PasswordInput" onChange={e => this.updatePasswordValue(e)} placeholder="Enter Password" />
              </FormGroup>
              <Button onClick={this.handleAuth} className="input-form-comp">Submit</Button>
            </Form>
          </div>
        );
      }
}

export default Login
