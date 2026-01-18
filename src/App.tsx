import { useState } from "react";
import "./assets/styles/tailwind.css";
import SideBar from "./components/Sidebar/SideBar";
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';

function App() {
    const [backendMessage, setBackendMessage] = useState("");
    const [filePath, setfilePath] = useState("No file selected");

    async function getBackendMessage(){
        setBackendMessage( await invoke("process_excel", { filePath: filePath }) );
    }

    async function getAllowedExtensions(){
        const allowedExtensions = await invoke("get_allowed_extensions");
        return allowedExtensions as string[];
    }

    async function selectFile() {
        const allowedExtensions = await getAllowedExtensions();

        const selected = await open({
            multiple: false,
            filters: [
                { name: 'Excel', extensions: allowedExtensions }
            ]
        })
        if (selected !== null) {
            setfilePath(selected);
        }
    }

    return (
        <main className="font-family-manrope bg-primary-light text-primary-dark flex w-full h-full gap-2">
            <SideBar/>
            {/* Receive excel from user */}
            <div>
                <div className="flex flex-col items-center justify-center p-6 border-2 border-dashed border-zinc-300 rounded-lg bg-white hover:border-blue-400 transition-colors">
                    <button 
                        onClick={ selectFile}
                        className="cursor-pointer bg-blue-600 text-white px-4 py-2 rounded-md hover:bg-blue-700 shadow-md"
                    >
                        Upload Excel File
                    </button>
                    <p className="mt-2 text-sm text-zinc-500">{filePath}</p>
                    <p className="mt-2 text-sm text-zinc-500">Only .xlsx, .xls, or .csv allowed</p>
                </div>
                <button 
                    className="mt-4 bg-green-600 text-white px-4 py-2 rounded-md hover:bg-green-700 shadow-md cursor-pointer"
                    onClick={getBackendMessage}
                >
                    Process File
                </button>
                <p className="mt-4 text-zinc-700">Backend Response: {backendMessage}</p>

            </div>
        </main>
    )
}

export default App;