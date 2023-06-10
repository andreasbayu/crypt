import { useEffect } from "react";
import { useState } from "react";
import { Tabs, TabList, Tab, TabPanel } from "react-tabs";
import algorithms from "./assets/algorithm_dict";
import * as libs from "./../libs/pkg/libs";

function App() {
  const [tabSelect, setTabSelect] = useState(0);

  const [selectedAlgorithmEncode, setSelectedAlgorithmEncode] = useState({
    index: 0,
    id: "",
    name: "",
    key_required: true,
    key_name: "",
  });
  const [selectedAlgorithmDecode, setSelectedAlgorithmDecode] = useState({
    index: 0,
    id: "",
    name: "",
    key_required: true,
    key_name: "",
  });

  const [algo, setAlgo] = useState([...algorithms]);

  const [plaintextEncode, setPlaintTextEncode] = useState("");
  const [plaintextDecode, setPlaintTextDecode] = useState("");
  const [keyEncode, setKeyEncode] = useState("");
  const [keyDecode, setKeyDecode] = useState("");
  const [encoded, setEncoded] = useState("");
  const [decoded, setDecoded] = useState("");

  function onEncode() {
    const id = selectedAlgorithmEncode.id;
    if (id === "caesar_chiper") {
      const exec = new Promise((resolve, reject) => {
        resolve(libs.caesar_encode(plaintextEncode, Number(keyEncode)));
      });
      exec
        .then((val) => setEncoded(val))
        .catch((err) => alert(`Error: ${err}`));
    } else if (id === "vigenere") {
      const exec = new Promise((resolve, reject) => {
        resolve(libs.vigenere_encode(plaintextEncode, keyEncode));
      });
      exec
        .then((val) => setEncoded(val))
        .catch((err) => alert(`Error: ${err}`));
    }
  }

  function onDecode() {
    const id = selectedAlgorithmEncode.id;
    if (id === "caesar_chiper") {
      const exec = new Promise((resolve, reject) => {
        resolve(libs.caesar_decode(plaintextDecode, Number(keyDecode)));
      });
      exec
        .then((val) => setDecoded(val))
        .catch((err) => alert(`Error: ${err}`));
    } else if (id === "vigenere") {
      const exec = new Promise((resolve, reject) => {
        resolve(libs.vigenere_decode(plaintextDecode, keyDecode));
      });
      exec
        .then((val) => setDecoded(val))
        .catch((err) => alert(`Error: ${err}`));
    }
  }

  useState(() => {
    setSelectedAlgorithmEncode(algo[0]);
    setSelectedAlgorithmDecode(algo[0]);
  });

  return (
    <div>
      <nav className="bg-white border-gray-200 shadow px-2 sm:px-4 py-2.5 rounded dark:bg-gray-900">
        <div className="container flex flex-wrap items-center justify-between mx-auto sm:w-4/6">
          <a href="https://flowbite.com/" className="flex items-center">
            <img
              src="https://cdn-icons-png.flaticon.com/512/391/391893.png"
              className="h-6 mr-4 sm:h-9"
              alt="Flowbite Logo"
            />
            <span className="self-center text-xl font-semibold whitespace-nowrap dark:text-white">
              Hashin
            </span>
          </a>
          <button
            data-collapse-toggle="navbar-default"
            type="button"
            className="inline-flex items-center p-2 ml-3 text-sm text-gray-500 rounded-lg md:hidden hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-gray-200 dark:text-gray-400 dark:hover:bg-gray-700 dark:focus:ring-gray-600"
            aria-controls="navbar-default"
            aria-expanded="false"
          >
            <span className="sr-only">Open main menu</span>
            <svg
              className="w-6 h-6"
              aria-hidden="true"
              fill="currentColor"
              viewBox="0 0 20 20"
              xmlns="http://www.w3.org/2000/svg"
            >
              <path
                fillRule="evenodd"
                d="M3 5a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zM3 10a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zM3 15a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1z"
                clipRule="evenodd"
              ></path>
            </svg>
          </button>
          <div className="hidden w-full md:block md:w-auto" id="navbar-default">
            <ul className="flex flex-col p-4 mt-4 border border-gray-100 rounded-lg bg-gray-50 md:flex-row md:space-x-8 md:mt-0 md:text-sm md:font-medium md:border-0 md:bg-white dark:bg-gray-800 md:dark:bg-gray-900 dark:border-gray-700">
              <li>
                <a
                  href="#"
                  className="block py-2 pl-3 pr-4 text-white bg-blue-700 rounded md:bg-transparent md:text-blue-700 md:p-0 dark:text-white"
                  aria-current="page"
                >
                  Text Encryption
                </a>
              </li>
              <li>
                <a
                  href="#"
                  className="block py-2 pl-3 pr-4 text-gray-700 rounded hover:bg-gray-100 md:hover:bg-transparent md:border-0 md:hover:text-blue-700 md:p-0 dark:text-gray-400 md:dark:hover:text-white dark:hover:bg-gray-700 dark:hover:text-white md:dark:hover:bg-transparent"
                >
                  File Encryption
                </a>
              </li>
              <li>
                <a
                  href="#"
                  className="block py-2 pl-3 pr-4 text-gray-700 rounded hover:bg-gray-100 md:hover:bg-transparent md:border-0 md:hover:text-blue-700 md:p-0 dark:text-gray-400 md:dark:hover:text-white dark:hover:bg-gray-700 dark:hover:text-white md:dark:hover:bg-transparent"
                >
                  About
                </a>
              </li>
            </ul>
          </div>
        </div>
      </nav>
      <div className="sm:flex justify-center h-full bg-cover mx-3 sm:mx-0">
        <div className="sm:w-1/3 mt-10">
          <div>
            <h1 className="text-center text-2xl text-gray-600 mb-2">Hashin</h1>
            <h1 className="text-center text-ll text-gray-600">
              Simple, fast, FREE encryption.
            </h1>
          </div>

          <div className="mt-10">
            <Tabs defaultIndex={0} onSelect={(index) => setTabSelect(index)}>
              <TabList className="grid grid-cols-2 gap-2 bg-gray-100 rounded p-2">
                <Tab>
                  <button
                    className={
                      "self-stretch w-full p-3 text-center  rounded-md " +
                      (tabSelect === 0
                        ? "bg-white text-black"
                        : "text-gray-500")
                    }
                  >
                    Encryption
                  </button>
                </Tab>
                <Tab>
                  <button
                    className={
                      "self-stretch w-full p-3 text-center rounded-md " +
                      (tabSelect === 1
                        ? "bg-white text-black"
                        : "text-gray-500")
                    }
                  >
                    Decryption
                  </button>
                </Tab>
              </TabList>
              <div className="mt-8">
                <TabPanel>
                  <div>
                    <div>
                      <select
                        className="bg-gray-300 rounded focus:outline-none w-full text-gray-700 px-5 py-3"
                        onChange={(e) => {
                          setSelectedAlgorithmEncode(algo[e.target.value]);
                        }}
                        value={selectedAlgorithmEncode.index}
                        key={"id"}
                      >
                        {algo.map((val) => (
                          <option
                            key={val.id}
                            className="bg-white text-gray-700"
                            id={val.id}
                            value={val.index}
                          >
                            {val.name}
                          </option>
                        ))}
                      </select>
                    </div>
                    <div className="mt-5">
                      <textarea
                        rows={8}
                        value={plaintextEncode}
                        onChange={(e) =>
                          setPlaintTextEncode(`${e.target.value}`)
                        }
                        className="px-6 py-5 w-full border-2 border-gray-200 focus:outline-none shadow-sm rounded-md"
                        placeholder="Paste plaintext here...."
                      />
                    </div>
                    <div
                      className={
                        "mt-5 " +
                        (!selectedAlgorithmEncode.key_required ? "hidden" : "")
                      }
                    >
                      <input
                        className="px-5 py-3 w-full border-2 border-gray-200 focus:outline-none shadow-sm rounded-md "
                        onChange={(e) => setKeyEncode(e.target.value)}
                        type="text"
                        name="key"
                        id="key"
                        value={keyEncode}
                        placeholder={selectedAlgorithmEncode.key_name}
                      />
                    </div>
                    <div className="mt-6 sm:flex sm:flex-row-reverse">
                      <button
                        className="self-stretch sm:w-1/4 w-full p-2 text-center bg-gray-400 text-white text-lg rounded-md"
                        onClick={onEncode}
                      >
                        Encode
                      </button>
                    </div>
                  </div>
                  <div className="mt-6 mb-20">
                    <textarea
                      value={encoded}
                      rows={10}
                      className="px-8 py-5 w-full border-0 focus:outline-none shadow-md rounded-md"
                      disabled
                    />
                  </div>
                </TabPanel>

                {/* Decode Tab */}
                <TabPanel>
                  <div>
                    <div>
                      <select
                        className="bg-gray-300 rounded focus:outline-none w-full text-gray-700 px-5 py-3"
                        onChange={(e) => {
                          setSelectedAlgorithmDecode(algo[e.target.value]);
                        }}
                        value={selectedAlgorithmDecode.index}
                      >
                        {algo.map((val, index) => (
                          <option
                            key={val.id}
                            className="bg-white text-gray-700"
                            id={val.id}
                            value={index}
                          >
                            {val.name}
                          </option>
                        ))}
                      </select>
                    </div>
                    <div className="mt-5">
                      <textarea
                        rows={8}
                        value={plaintextDecode}
                        onChange={(e) =>
                          setPlaintTextDecode(`${e.target.value}`)
                        }
                        className="px-6 py-5 w-full border-2 border-gray-200 focus:outline-none shadow-sm rounded-md"
                        placeholder="Paste plaintext here...."
                      />
                    </div>
                    <div
                      className={
                        "mt-5 " +
                        (!selectedAlgorithmDecode.key_required ? "hidden" : "")
                      }
                    >
                      <input
                        className="px-5 py-3 w-full border-2 border-gray-200 focus:outline-none shadow-sm rounded-md "
                        onChange={(e) => setKeyDecode(e.target.value)}
                        type="text"
                        name="key"
                        id="key"
                        value={keyDecode}
                        placeholder={selectedAlgorithmDecode.key_name}
                      />
                    </div>
                    <div className="mt-6 sm:flex sm:flex-row-reverse">
                      <button
                        className="self-stretch sm:w-1/4 w-full p-2 text-center bg-gray-400 text-white text-lg rounded-md"
                        onClick={onDecode}
                      >
                        Decode
                      </button>
                    </div>
                  </div>
                  <div className="mt-6 mb-20">
                    <textarea
                      value={decoded}
                      rows={10}
                      className="px-8 py-5 w-full border-0 focus:outline-none shadow-md rounded-md"
                      disabled
                    />
                  </div>
                </TabPanel>
              </div>
            </Tabs>
          </div>
        </div>
      </div>
    </div>
  );
}

export default App;
