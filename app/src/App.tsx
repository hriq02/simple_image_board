import { createResource, createSignal, ResourceReturn, type Component } from 'solid-js';

import logo from './logo.svg';
import styles from './App.module.css';
import TagsTab from './components/TagsTab';
import Header from './components/header';
import PostsPages from './components/PostsPages';
import { QueryReq } from './schemas';
import Footer from './components/footer';

async function fetchPosts(tags: string[], page: number = 1) {
  const query = `
    query($tags: [String!]!, $page: Int!) {
      queryPosts(tags: $tags, page: $page) {
        posts { id, uploader, artist, tags }
        tags { name }
      }
    }
  `;

  const res = await fetch("http://localhost:8000/query", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({
        query,
        variables: { tags, page }
    }),
  });

  if (!res.ok) {
      console.error("GraphQL request failed", await res.text());
      return null;
  }

  const json = await res.json();
  console.log(json.data.queryPosts);
  return json.data.queryPosts;
}

const App: Component = () => {
  const params = new URLSearchParams(window.location.search);
  const q = params.get("q") ?? "";
  const page = parseInt(params.get("page") ?? "1", 10);
  const tags = q.split(/\s+|\+/).filter(Boolean);
  const [search, setSearch] = createSignal("");
  
  const [postsData] : ResourceReturn<QueryReq,unknown> = createResource(
    () => ({ tags, page }),
    ({ tags, page }) => fetchPosts(tags, page)
  );
  return (
    <div>
      <Header search={search()} setSearch={setSearch}/>
      <div class="panels-father">
        <TagsTab 
          tags={postsData()?.tags} 
          onTagClick={(tag) => {
            setSearch((prev) => {
              const tagsArray = prev.split('+').filter(Boolean);
              if (!tagsArray.includes(tag)) {
                tagsArray.push(tag); // adiciona só se não existir
              }
              return tagsArray.join(' ');
            });
          }} 
        />
        <PostsPages posts={postsData()?.posts}/>
      </div>
      <Footer />
    </div>
  );
};

export default App;
