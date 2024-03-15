import './index.css';
import React, { useState } from "react";

const initialData = {
  id: 1,
  text: "heres a title that the user can edit",
  options: {
    "1": {
      text: "heres an option. the user can edit the next_id of this",
      next_id: 2,
    },
    "2": {
      text: "heres another option. the user can also edit the next_id of this",
      next_id: 3,
    },
  },
};

const App = () => {
  const [formData, setFormData] = useState({ ...initialData });
  const [cards, setCards] = useState([]);
  const [showJson, setShowJson] = useState(false);

  const handleChange = (e) => {
    const { name, value } = e.target;
    setFormData((prevData) => ({
      ...prevData,
      [name]: value,
    }));
  };

  const handleSubmit = (e) => {
    e.preventDefault();
    const newCard = { ...formData, id: getMaxId() + 1 };
    setCards((prevCards) => [...prevCards, newCard]);
  };

  const getMaxId = () => {
    return cards.reduce((maxId, card) => (card.id > maxId ? card.id : maxId), 0);
  };

  const handleOptionChange = (e, optionId) => {
    const { name, value } = e.target;
    setFormData((prevData) => ({
      ...prevData,
      options: {
        ...prevData.options,
        [optionId]: {
          ...prevData.options[optionId],
          [name]: value,
        },
      },
    }));
  };

  const toggleJsonView = () => {
    setShowJson((prevShowJson) => !prevShowJson);
  };

  return (
    <div className="flex">
      <div className="w-1/2 p-4">
        <form onSubmit={handleSubmit}>
          <label htmlFor="text" className="block mb-2">
            Title:
          </label>
          <input
            type="text"
            id="text"
            name="text"
            value={formData.text}
            onChange={handleChange}
            className="w-full border border-gray-300 rounded px-3 py-2 mb-4"
          />
          <button type="submit" className="bg-blue-500 text-white px-4 py-2 rounded">
            Add Card
          </button>
        </form>
      </div>
      <div className="w-1/2 p-4">
        <h2 className="text-lg font-semibold mb-4">Created Cards</h2>
        <button onClick={toggleJsonView} className="bg-blue-500 text-white px-4 py-2 rounded mt-4">
          {showJson ? "Back to Cards" : "View JSON"}
        </button>
        {showJson ? (
          <pre>{JSON.stringify({ entries: cards }, null, 2)}</pre>
        ) : (
          <div>
            {cards.map((card) => (
              <div key={card.id} className="bg-gray-100 p-3 rounded mb-3">
                <h3 className="font-semibold">{card.text}</h3>
                <ul>
                  {Object.keys(card.options).map((optionId) => (
                    <li key={optionId} className="mb-2">
                      <input
                        type="text"
                        value={card.options[optionId].text}
                        onChange={(e) => handleOptionChange(e, optionId)}
                        name="text"
                        className="border border-gray-300 rounded px-2 py-1"
                      />
                      <label className="ml-2">Next ID:</label>
                      <input
                        type="number"
                        value={card.options[optionId].next_id}
                        onChange={(e) => handleOptionChange(e, optionId)}
                        name="next_id"
                        className="border border-gray-300 rounded px-2 py-1"
                      />
                    </li>
                  ))}
                </ul>
              </div>
            ))}
          </div>
        )}
       
      </div>
    </div>
  );
};

export default App;
