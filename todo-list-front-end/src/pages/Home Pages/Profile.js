import React from 'react';
import css from './css/Profile.module.css';

function Profile(props) {
    return (
        <div className={css.profile}>
            <h4> accuracy  :  95% </h4>{/* The tasks completed with in the time*/}
            <h4> today tasks   :  12 </h4>
            <h4> pending tasks  :  1 </h4>{/** The number of pending tasks today */}
        </div>
    );
}

export default Profile;