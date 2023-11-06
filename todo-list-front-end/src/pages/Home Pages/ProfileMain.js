import React from 'react';
import maleUser from "../../account.png";
import femaleUser from "../../profile-picture.png";
import css from "./css/Profile.module.css";

function ProfileMain(props) {
    return (
        <div className={css.profile}>
            <div className={css.profileedit}>
            <img src={props.sex === 'female' ? femaleUser : maleUser} alt='user' />
            <button>
                edit
            </button>
            </div>
            <h5> username: phani </h5>
            <h5> mobilenumber: 8885858760 </h5>
            <h5> Email : phanidharguttikonda0@gmail.com</h5>
        </div>
    );
}

export default ProfileMain;