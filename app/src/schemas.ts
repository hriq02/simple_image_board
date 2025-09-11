

export enum TagType{
    ARTIST,
    USER,
    
    IMAGE,
    VIDEO,
    GIF,

    CHARACTER,
    COPYRIGHT,

    DEFAULT
}


export function tag_to_type(tag: Tag) : TagType{
    if (!tag.tagType){
        return TagType.DEFAULT;
    }
    switch(tag.tagType){
        case "A":
            return TagType.ARTIST;
        case "U":
            return TagType.USER;
        case "I":
            return TagType.IMAGE;
        case "V":
            return TagType.VIDEO;
        case "G":
            return TagType.GIF;
        case "C":
            return TagType.CHARACTER;
        case "c":
            return TagType.COPYRIGHT;
        default:
            return TagType.DEFAULT;
    }
}


export interface Post {
    id: number;
    uploader: string;
    artist: string;
    tags: Tag[];
}


export interface Tag {
    name: string;
    tagType: string;
}

export interface QueryReq{
    tags: Tag[];
    posts: Post[] | undefined;
}



