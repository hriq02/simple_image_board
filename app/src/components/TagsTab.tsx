import { For, type Component } from 'solid-js';
import { Tag, tag_to_type, TagType } from '../schemas';


function get_color_type(type: TagType): string {
  
    switch (type) {
      case TagType.ARTIST:
        return "color: red;";
      case TagType.USER:
        return "color: orange;";
      case TagType.IMAGE:
        return "color: #67b1ff;";
      case TagType.VIDEO:
        return "color: blue;";
      case TagType.GIF:
        return "color: #8a58ff;";
      case TagType.CHARACTER:
        return "color: green;";
      case TagType.COPYRIGHT:
        return "color: rgb(119, 128, 0)";
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