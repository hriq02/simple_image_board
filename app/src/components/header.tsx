import type { Component } from 'solid-js';


function format_search(search: string) : string{
    return `http://localhost:3000/?q=${search.replaceAll(" ", "+")}`;
}

const Header: Component<{
  search: string;
  setSearch: (s: string) => void;
}> = (props) => {

  const handleInput = (e: Event) => {
    const target = e.target as HTMLInputElement;
    props.setSearch(target.value);
  };

  return (
    <div>
      <header>
        <h1>Header</h1>
      </header>
      
      <div>
        <input 
          type="text" 
          placeholder="Search" 
          value={props.search} 
          onInput={handleInput} 
        />
        <button
            onClick={() => window.location.href = format_search(props.search)}
        >...
        </button>
      </div>
    </div>
  );
};

export default Header;
