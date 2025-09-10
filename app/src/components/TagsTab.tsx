import { For, type Component } from 'solid-js';
import { Tag, tag_to_type, TagType } from '../schemas';


function get_color_type(type: TagType): string {
  
    switch (type) {
      case TagType.ARTIST:
        return "color: red;";
      case TagType.USER:
        return "color: blue;";
      case TagType.IMAGE:
        return "color: yellow;";
      case TagType.VIDEO:
        return "color: orange;";
      case TagType.GIF:
        return "color: purple;";
      case TagType.CHARACTER:
        return "color: pink;";
      case TagType.COPYRIGHT:
        return "color: brown;";
      case TagType.DEFAULT:
        return "color: green;";
      default:
        return "color: green;";
    }
}


const TagsTab: 
    Component<{
        tags: Tag[] | undefined, 
        onTagClick?: (tag: string) => void
    }> = (tags_) => {
    return (
        <div class="tags-panel">
            <ul>
                <For each={tags_.tags ?? []}>
                    {(tag) => (
                        <li 
                            class="tag"
                            onClick={() => tags_.onTagClick?.(tag.name)}
                            style={get_color_type(tag_to_type(tag))}>{tag.name}
                        </li>
                    )}
                </For>
            </ul>
        </div>
    );
};


export default TagsTab;