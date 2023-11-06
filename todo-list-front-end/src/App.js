import React, { useEffect, useState } from "react";
import { BrowserRouter, Route, Routes } from "react-router-dom";
import "./App.css";
import Home from "./pages/Home";
import SignIn from "./pages/SignIn";
import SignUp from "./pages/SignUp";
import CreateTask from "./pages/tasks/CreateTask";

export const usernameContext = React.createContext();
export const usernamecheckContest = React.createContext();
export const completedtasksContext = React.createContext();
export const completedtaskscheck = React.createContext();
export const InCompletedtasksContext = React.createContext();
export const InCompletetasksCheck = React.createContext();
export const currenttasksContext = React.createContext();
export const currenttasksCheck = React.createContext();

const tasks = [
  {
    taskname: "phani",
    starttime: "12:35",
    endtime: "3:05",
  },
  {
    taskname: "example",
    starttime: "9:00",
    endtime: "11:30",
  },
  {
    taskname: "meeting",
    starttime: "2:00",
    endtime: "4:30",
  },
  // Add more objects below
  {
    taskname: "task1",
    starttime: "8:00",
    endtime: "9:30",
  },
  {
    taskname: "task2",
    starttime: "10:00",
    endtime: "12:30",
  },
  {
    taskname: "task3",
    starttime: "1:30",
    endtime: "2:45",
  },
  {
    taskname: "task4",
    starttime: "4:00",
    endtime: "5:15",
  },
  {
    taskname: "task5",
    starttime: "6:00",
    endtime: "7:30",
  },
  {
    taskname: "task6",
    starttime: "8:30",
    endtime: "9:45",
  },
  {
    taskname: "task7",
    starttime: "10:30",
    endtime: "11:45",
  },
  {
    taskname: "task8",
    starttime: "1:00",
    endtime: "2:15",
  },
  {
    taskname: "task9",
    starttime: "3:30",
    endtime: "4:45",
  },
  {
    taskname: "task10",
    starttime: "5:30",
    endtime: "6:45",
  },
];

const task2 = [
  {
    taskname: "task8",
    starttime: "1:00",
    endtime: "2:15",
  },
  {
    taskname: "task9",
    starttime: "3:30",
    endtime: "4:45",
  },
  {
    taskname: "task10",
    starttime: "5:30",
    endtime: "6:45",
  },
];

const task3 = [
  {
    taskname: "task2",
    starttime: "10:00",
    endtime: "12:30",
  },
  {
    taskname: "task3",
    starttime: "1:30",
    endtime: "2:45",
  },
  {
    taskname: "task4",
    starttime: "4:00",
    endtime: "5:15",
  },
  {
    taskname: "task5",
    starttime: "6:00",
    endtime: "7:30",
  },
  {
    taskname: "task6",
    starttime: "8:30",
    endtime: "9:45",
  },
  {
    taskname: "task7",
    starttime: "10:30",
    endtime: "11:45",
  },
];

function App() {
  // here we are going to manage the state of the react components

  const [username, changeUserName] = useState("");
  const [completedtasks, changecompletedtaks] = useState([]);
  const [incompletetasks, changeincompletetasks] = useState([]);
  const [currenttasks, changecurrenttasks] = useState([]);

  useEffect(() => {
    // now we are going to fetch the completed tasks or un completed tasks or current tasks
  }, [username]);

  return (
    <BrowserRouter>
      <currenttasksContext.Provider value={changecurrenttasks}>
        <currenttasksCheck.Provider value={currenttasks}>
          <InCompletedtasksContext.Provider value={changeincompletetasks}>
            <InCompletetasksCheck.Provider value={incompletetasks}>
              <completedtasksContext.Provider value={changecompletedtaks}>
                <completedtaskscheck.Provider value={completedtasks}>
                  <usernamecheckContest.Provider value={username}>
                    <usernameContext.Provider value={changeUserName}>
                      <div className={"App"}>
                        <Routes>
                          <Route element={<Home array={tasks} />} path="/" />
                          {/**here we will pass current tasks */}
                          <Route element={<SignIn />} path="/Sign-In" />
                          <Route element={<SignUp />} path="/Sign-Up" />
                          <Route path="/create-task" element={<CreateTask />} />
                          <Route
                            path="/completed-tasks"
                            element={<Home array={task2} />}
                          />
                          <Route
                            path="/incompleted-tasks"
                            element={<Home array={task3} />}
                          />
                        </Routes>
                      </div>
                    </usernameContext.Provider>
                  </usernamecheckContest.Provider>
                </completedtaskscheck.Provider>
              </completedtasksContext.Provider>
            </InCompletetasksCheck.Provider>
          </InCompletedtasksContext.Provider>
        </currenttasksCheck.Provider>
      </currenttasksContext.Provider>
    </BrowserRouter>
  );
}

export default App;
