import React from 'react';
import css from './css/Body.module.css';

function Body(props) {
    return (
        <div className={css.body}>
            {
                props.array.map((item, index) => {
                    return <div className={css.task}> 
                        <div className={css.uppertask}> 
                            <h3> {item.taskname} </h3>
                            <button> delete </button>
                        </div>
                        <div className={css.lowertask}> 
                            <h4> start-time : {item.starttime} </h4>
                            <h4> end-time : {item.endtime} </h4>
                            <button> completed </button>
                        </div>
                    </div>
                })
            }
        </div>
    );
}

export default Body;