import React, { useContext } from 'react';
import { Link } from 'react-router-dom';
import { usernamecheckContest } from '../App';
import Body from './Home Pages/Body';
import Sidenav from './Home Pages/Sidenav';
import css from "./css/Home.module.css";

  



function Home(props) {

    const username = useContext(usernamecheckContest) ;
    


    return (
          <div className={css.home}>
            {
                username.length === 0 ? <div className={css.no}> 
                    <Link to='/Sign-Up' className={css.link}> Sign Up </Link>
                    <h3> or </h3>
                    <Link to='/Sign-In' className={css.link}> Sign In</Link>
                </div> : <div className={css.main}>
                <Sidenav className={css.Sidenav}/>
                <Body className={css.Body} array={props.array}/>
                </div>
            }

        </div>
    );
}

export default Home;