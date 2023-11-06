import React from 'react';
import { Link } from 'react-router-dom';
import Profile from './Profile';
import ProfileMain from './ProfileMain';
import css from "./css/Sidenav.module.css";

function Sidenav(props) {
    return (
        <div className={css.nav}>
            <Profile />
            <div className={css.menu}>
                <button><Link to={'/'} className={css.link}>Home</Link></button>
                <button><Link to={'/create-task'} className={css.link}>create task</Link></button>
                <button><Link to={'/completed-tasks'} className={css.link}>completed-tasks</Link></button>
                <button><Link to={'/incompleted-tasks'} className={css.link}>uncompleted-tasks</Link></button>
            </div>
            <ProfileMain />
        </div>
    );
}

export default Sidenav;