import type { Component } from 'solid-js';
import { createResource, For, Show, createSignal, onCleanup } from 'solid-js';
import { Post } from '../schemas';

// --- Função util para HEAD request ---
async function getMimeType(url: string): Promise<string | null> {
  try {
    const res = await fetch(url, { method: "HEAD" });
    return res.ok ? res.headers.get("Content-Type") : null;
  } catch {
    return null;
  }
}


function useVideoThumbnail(url: string) {
  const [thumb, setThumb] = createSignal<string | null>(null);
  const video = document.createElement("video");
  video.src = url;
  video.crossOrigin = "anonymous";
  video.muted = true;
  video.preload = "metadata";
  video.style.display = "none";
  document.body.appendChild(video);

  const onLoaded = () => {
    const canvas = document.createElement("canvas");
    canvas.width = video.videoWidth;
    canvas.height = video.videoHeight;
    const ctx = canvas.getContext("2d")!;
    ctx.drawImage(video, 0, 0, canvas.width, canvas.height);
    setThumb(canvas.toDataURL("image/jpeg"));
    video.remove();
  };

  video.addEventListener("loadedmetadata", onLoaded);
  onCleanup(() => video.remove());

  return thumb;
}

const PostsPages: Component<{ posts: Post[] | undefined}> = (props) => {
  return (
    <div>
      <ul class="thumbs_grid">
        <For each={props.posts ?? []}>
          {(post) => {
            const url = `http://localhost:7000/${post.id}`;
            const [mime] = createResource(() => getMimeType(url));
            const thumb = useVideoThumbnail(url);
            const onClick = () => window.open(url, "_blank");

            return (
              <Show
                when={mime()?.startsWith("video/")}
                fallback={<img src={url} class="thumb" onClick={onClick}/>}
              >
                <Show when={thumb()} fallback={<div class="thumb placeholder" />}>
                  <img src={thumb()!} class="thumb" onClick={onClick}/>
                </Show>
              </Show>
            );
          }}
        </For>
      </ul>
    </div>
  );
};

export default PostsPages;
