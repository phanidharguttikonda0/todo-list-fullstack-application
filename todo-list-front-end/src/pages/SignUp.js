import React, { useContext, useState } from 'react';
import { Link, useNavigate } from 'react-router-dom';
import { usernameContext } from '../App';
import css from './css/Login.module.css';

function SignUp(props) {

    const [username, changeUserName] = useState("") ;
    const [mobileNumber, changeMobileNumber] = useState("") ;
    const [email, changeEmail] = useState("") ;
    const [password, changePassword] = useState("") ;
    const usernameSet = useContext(usernameContext) ;
    const navigate = useNavigate() ;

    const setEmail = (event) =>{
        changeEmail(event.target.value) ;
    }

    const setMobile = (event) =>{
        changeMobileNumber(event.target.value) ;
    }

    const setUsername = (event) => {
        changeUserName(event.target.value) ;
    }

    const setPassword = (event) => {
        changePassword(event.target.value) ;
    }

    const submit = () => {
        // first we need to check whether the Signup details are not in the database
        usernameSet(username) ;
        navigate('/') ;
    }

    return (
        <div className={css.up}>
            <input type='email' placeholder='Email' value={email} onChange={setEmail} />
            <input type='number' placeholder='Mobile-Number' value={mobileNumber} onChange={setMobile} />
            <input type='text' placeholder='user-name' value={username} onChange={setUsername} />
            <input type='password' placeholder='password' value={password} onChange={setPassword} />
            <Link to='/Sign-In' className={css.Link}>
                SignIn
            </Link>
            <button onClick={submit}> sign up </button>
        </div>
    );
}

export default SignUp;