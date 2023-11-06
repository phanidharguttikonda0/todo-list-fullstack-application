import React, { useContext, useState } from 'react';
//import ReactDOM from 'react-dom';
import { useNavigate } from 'react-router-dom';
import { usernamecheckContest } from '../../App';
import Home from '../Home';
import css from './css/CreateTask.module.css';




function CreateTask(props) {
    const username = useContext(usernamecheckContest) ;
    const [starttime, changestarttime] = useState("") ;
    const [endtime, changeendtime]  = useState("") ;
    const [taskname, changetaskname] = useState("") ;
    const navigate = useNavigate() ;

    const starttimehandler = (event) => {
        changestarttime(event.target.value) ;
    }

    const endtimehandler = (event) => {
        changeendtime(event.target.value) ;
    }

    const tasknamehandler = (event) =>{
        changetaskname(event.target.value) ;
    }

    const onsubmit = () => {
        // here we will check and then we will navigate
        navigate('/') ;
    }

    const close = () => {
        navigate('/') ;
    }

    return (
        <div className={css.new}>
            {
                username.length === 0 ? <Home /> : <div> 
                    <input type='text' value={taskname} onChange={tasknamehandler}/>
                <input type='time' value={starttime} onChange={starttimehandler}/>
                <input type='time' value={endtime} onChange={endtimehandler}/>
                <div className={css.btn}>
                <button onClick={onsubmit}> continue</button>
                <button onClick={close} > close </button>
                </div>
                </div>
            }{
                console.log(username.length)
            }
        </div>
    );
}

export default CreateTask;