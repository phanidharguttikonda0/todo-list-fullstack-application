import React, { useContext, useState } from 'react';
import { Link, useNavigate } from 'react-router-dom';
import { usernameContext } from '../App';
import css from "./css/Login.module.css";

function SignIn(props) {

    const usernameSet = useContext(usernameContext) ;
    const [username, changeUserName] = useState("") ;
    const [password, changePassword] = useState("") ;
    const navigate = useNavigate() ;

    const submit = () =>{
        // we will check whether the username is there or not if there we will check whether the password matching
        usernameSet(username) ;
        navigate('/') ;
    }

    const setusername = (event) =>{
        changeUserName(event.target.value) ;
    }

    const setpassword = (event) =>{
        changePassword(event.target.value) ;
    }

    return (
        <div className={css.up}>
            <input type='text' placeholder='username' value={username} onChange={setusername}/>
            <input type='password' placeholder='password' value={password} onChange={setpassword}/>
            <Link to='/Sign-Up' className={css.Link}>
                SignUp
            </Link>
            <button onClick={submit}> sign in </button>
        </div>
    );
}

export default SignIn;