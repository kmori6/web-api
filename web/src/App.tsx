import "./App.css";

function App() {
  const handler = async () => {
    const response = await fetch(import.meta.env.VITE_URL);
    if (!response.ok) {
      throw Error("Failed to fetch from api.");
    }
    console.log(await response.text());
  };

  return (
    <>
      <button onClick={handler}>send</button>
    </>
  );
}

export default App;
