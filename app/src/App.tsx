import type { Component } from 'solid-js';

import logo from './logo.svg';
import styles from './App.module.css';
import TagsTab from './components/TagsTab';
import Header from './components/header';
import PostsPages from './components/PostsPages';

const App: Component = () => {
  return (
    <div>
      <Header />
      <div>
        <TagsTab />
        <PostsPages />
      </div>
    </div>
  );
};

export default App;
