import React, { useState } from "react";

const initialData = {
  id: 1,
  text: "here's an option text. this is where you can ask the player something or give them information.",
  options: {
    "1": {
      text: "here's an option.",
      next_id: 2,
    },
    "2": {
      text: "here's another option. ",
      next_id: 3,
    },
  },
};

const App = () => {
  const [title, setTitle] = useState("An Unnamed Story");
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

  const addCard = (e) => {
    const newCard = { ...formData, id: getMaxId() + 1 };
    setCards((prevCards) => [...prevCards, newCard]);
  };

  const getMaxId = () => {
    return cards.reduce((maxId, card) => (card.id > maxId ? card.id : maxId), 0);
  };

  const handleOptionChange = (e, optionId, cardId) => {
    const { name, value } = e.target;
    setCards((prevCards) =>
      prevCards.map((card) => {
        if (card.id === cardId) {
          return {
            ...card,
            options: {
              ...card.options,
              [optionId]: {
                ...card.options[optionId],
                [name]: value,
              },
            },
          };
        }
        return card;
      })
    );
  };

  const handleTitleChange = (e, cardId) => {
    const { value } = e.target;
    setCards((prevCards) =>
      prevCards.map((card) => {
        if (card.id === cardId) {
          return {
            ...card,
            text: value,
          };
        }
        return card;
      })
    );
  };

  const toggleJsonView = () => {
    setShowJson((prevShowJson) => !prevShowJson);
  };

  const deleteCard = (id) => {
    setCards((prevCards) => prevCards.filter((card) => card.id !== id));
  };

  const handleEndingChange = (e, cardId) => {
    const isChecked = e.target.checked;
    setCards((prevCards) =>
      prevCards.map((card) => {
        if (card.id === cardId) {
          return {
            ...card,
            options: isChecked ? {} : initialData.options,
          };
        }
        return card;
      })
    );
  };
 const handleChangeTitle = (e) => {
    setTitle(e.target.value);
  };

  const downloadJson = () => {
    const json = JSON.stringify({ title: title, entries: cards }, null, 2);
    const blob = new Blob([json], { type: "application/json" });
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;

   if (title) {
  let fixedTitle = title.trim().toLowerCase().replace(/ /g, "_");
  a.download = fixedTitle + ".json";
} else {
  a.download = "data.json";
}

    a.click();
    URL.revokeObjectURL(url);
  };


  return (
    <div className="flex flex-row justify-center h-screen w-screen bg-stone-900">
      <div className="w-1/2 p-4 bg-stone-500 text-white">
        <button onClick={addCard} className="bg-stone-600 border-2 border-white text-white px-4 py-2 mr-2 rounded">
          Add Card
        </button>
        <button onClick={toggleJsonView} className="bg-stone-600 border-2 border-white text-white px-4 py-2 rounded mt-4">
          {showJson ? "View Cards" : "View JSON"}
        </button>
        <button onClick={downloadJson} className="bg-stone-600 border-2 border-white text-white px-4 ml-2 py-2 rounded mt-4">
          Download
        </button>
         <input
          type="text"
          value={title}
          onChange={handleChangeTitle}
          className="font-semibold text-xl bg-stone-600 border-2 border-white text-white rounded px-2 py-1 w-full mt-2"
          placeholder="Enter a cool title..."
        />
        {showJson ? (
          <>
          <h1 className="text-md text-white mt-2">You can manually edit this data, but it isn't recommended unless you know what you're doing.</h1>
            <pre contentEditable={true} className="overflow-y-auto h-screen max-h-full bg-stone-600 border-2 border-white text-white rounded px-4 py-2">
              {JSON.stringify({ title: title, entries: cards }, null, 2)}
            </pre>
          </>
        ) : (
          <div className="overflow-y-auto h-screen max-h-full bg-stone-600 border-2 border-white text-white rounded px-4 py-2 mt-4">
            {cards.map((card) => (
              <div key={card.id} className="bg-stone-500 border border-gray-300 p-3 rounded mt-4">
                <pre>ID: {card.id}</pre>
                <input
                  type="text"
                  value={card.text}
                  onChange={(e) => handleTitleChange(e, card.id)}
                  className="font-semibold text-xl bg-stone-600 border-2 border-white text-white rounded px-2 py-1 w-full mb-2"
                  placeholder="Enter card text..."
                />
                <label>
                  Ending:
                  <input
                    type="checkbox"
                    checked={Object.keys(card.options).length === 0}
                    onChange={(e) => handleEndingChange(e, card.id)}
                    className="ml-2 bg-stone-600 border-2 border-white text-white"
                  />
                </label>
                <ul>
                  {Object.keys(card.options).map((optionId) => (
                    <li key={optionId} className="mb-2 w-full transition-all duration-300">
                      <input
                        type="text"
                        value={card.options[optionId].text}
                        onChange={(e) => handleOptionChange(e, optionId, card.id)}
                        name="text"
                        className="bg-stone-600 border-2 border-white text-white rounded px-2 py-1 w-96"
                        placeholder="Enter option text..."
                      />
                      <label className="ml-2">Next ID:</label>
                      <input
                        type="number"
                        value={card.options[optionId].next_id}
                        onChange={(e) => handleOptionChange(e, optionId, card.id)}
                        name="next_id"
                        className="bg-stone-600 border-2 border-white text-white rounded px-2 py-1 w-20"
                        placeholder="Next ID"
                      />
                    </li>
                  ))}
                </ul>
                <button onClick={() => deleteCard(card.id)} className="bg-red-500 text-white px-2 py-1 rounded">
                  Delete
                </button>
              </div>
            ))}
          </div>
        )}
      </div>
    </div>
  );
};

export default App;
