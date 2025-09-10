import type { Component } from 'solid-js';



const TagsTab: Component = () => {
    const tags = ["teste1", "teste2"];
    return (
        <div>
            <ul>
                {tags.map((tag) => (
                    <li>{tag} + </li>
                ))}
            </ul>
        </div>
    );
};


export default TagsTab;