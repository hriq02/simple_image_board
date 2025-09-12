import { Component, createSignal, createEffect, For } from "solid-js";
import CreateTagModal from "./newTagModal";

const UploadModal: Component<{ toggleModal: () => void }> = (props) => {
  const [user, setUser] = createSignal("");
  const [artist, setArtist] = createSignal("");
  const [tagsInput, setTagsInput] = createSignal("");
  const [tagsSuggestions, setTagsSuggestions] = createSignal<string[]>([]);
  const [selectedTags, setSelectedTags] = createSignal<string[]>([]);
  const [file, setFile] = createSignal<File | null>(null);

  // Novo modal de criar tag
  const [newTagModalOpen, setNewTagModalOpen] = createSignal(false);
  const [newTagName, setNewTagName] = createSignal("");

  // Função para buscar tags no GraphQL
  const fetchTags = async (query: string) => {
    if (!query) return setTagsSuggestions([]);
    const response = await fetch("http://localhost:8000/query", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        query: `query { queryTag(nearTag:"${query}") { name } }`
      }),
    });
    const data = await response.json();
    const suggestions = data?.data?.queryTag?.map((t: any) => t.name) || [];
    setTagsSuggestions(suggestions);
  };

  createEffect(() => fetchTags(tagsInput()));

  const addTag = (tag: string) => {
    if (!selectedTags().includes(tag)) {
      setSelectedTags([...selectedTags(), tag]);
      setTagsInput("");
    }
  };

  // Abrir modal de criar tag se não existir
  const handleAddTagInput = () => {
    if (tagsInput() && !tagsSuggestions().includes(tagsInput())) {
      setNewTagName(tagsInput());
      setNewTagModalOpen(true);
    } else {
      addTag(tagsInput());
    }
  };

  const handleDrop = (e: DragEvent) => {
    e.preventDefault();
    const files = e.dataTransfer?.files;
    if (files && files[0]) setFile(files[0]);
  };

  const handleDragOver = (e: DragEvent) => e.preventDefault();

  const handleSubmit = async (e: Event) => {
    e.preventDefault();
  
    if (!file()) {
      alert("Selecione um arquivo antes de enviar.");
      return;
    }
  
    try {
      const formData = new FormData();
      formData.append("file", file()!);
  
      const uploadRes = await fetch("http://localhost:7000/", {
        method: "POST",
        body: formData,
      });
  
      if (!uploadRes.ok) {
        const msg = await uploadRes.text();
        alert("Erro no servidor de arquivos: " + msg);
        return;
      }
  
      const payload = {
        uploader: user(),
        artist: artist(),
        tags: selectedTags().map((t) => ({ name: t, tag_type: "" })), // exemplo
      };
  
      const apiRes = await fetch("http://localhost:8000/upload", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(payload),
      });
  
      if (!apiRes.ok) {
        const msg = await apiRes.text();
        alert("Erro na API: " + msg);
        return;
      }
  
      const created = await apiRes.json();
      console.log("Post criado:", created);
  
      props.toggleModal();
    } catch (err) {
      console.error(err);
      alert("Erro inesperado no upload");
    }

    window.location.href = "http://localhost:3000";
  };
  

  function newTagInitialName(): string | undefined {
    throw new Error("Function not implemented.");
  }


  return (
    <>
      {/* Modal principal */}
      <div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
        <div class="bg-white rounded-2xl shadow-lg w-11/12 max-w-lg p-6 relative">
          <button
            class="absolute top-4 right-4 text-gray-500 hover:text-gray-700"
            onClick={props.toggleModal}
          >✕</button>

          <h2 class="text-xl font-bold mb-4">Upload de Postagem</h2>

          <form class="flex flex-col gap-4" onSubmit={handleSubmit}>
            <input
              type="text"
              placeholder="User"
              class="border rounded p-2 focus:outline-none focus:ring-2 focus:ring-blue-500"
              value={user()}
              onInput={(e) => setUser(e.currentTarget.value)}
              required
            />
            <input
              type="text"
              placeholder="Artist"
              class="border rounded p-2 focus:outline-none focus:ring-2 focus:ring-blue-500"
              value={artist()}
              onInput={(e) => setArtist(e.currentTarget.value)}
              required
            />

            {/* Tags com autocomplete */}
            <div class="relative">
              <input
                type="text"
                placeholder="Tags"
                class="border rounded p-2 w-full focus:outline-none focus:ring-2 focus:ring-blue-500"
                value={tagsInput()}
                onInput={(e) => setTagsInput(e.currentTarget.value)}
                onKeyDown={(e) => {
                  if (e.key === "Enter") {
                    e.preventDefault();
                    handleAddTagInput();
                  }
                }}
              />
              {tagsSuggestions().length > 0 && (
                <ul class="absolute top-full left-0 right-0 bg-white border rounded shadow max-h-40 overflow-y-auto z-10">
                  <For each={tagsSuggestions()}>
                    {(tag) => (
                      <li
                        class="p-2 hover:bg-gray-200 cursor-pointer"
                        onClick={() => addTag(tag)}
                      >
                        {tag}
                      </li>
                    )}
                  </For>
                </ul>
              )}
              <div class="flex flex-wrap gap-2 mt-1">
                <For each={selectedTags()}>
                  {(tag) => (
                    <span class="bg-blue-200 text-blue-800 px-2 py-1 rounded-full text-sm">
                      {tag}
                    </span>
                  )}
                </For>
              </div>
            </div>

            {/* Drag & Drop */}
            <div
              class="border-2 border-dashed border-gray-400 rounded p-6 flex items-center justify-center text-gray-500 hover:border-gray-600 cursor-pointer"
              onDrop={handleDrop}
              onDragOver={handleDragOver}
            >
              {file()? file()?.name : "Arraste um arquivo aqui ou clique para selecionar"}
              <input
                type="file"
                class="hidden"
                onChange={(e) => setFile(e.currentTarget.files?.[0] || null)}
              />
            </div>

            <button
              type="submit"
              class="bg-blue-600 text-white py-2 px-4 rounded hover:bg-blue-700 transition"
            >
              Send
            </button>
          </form>
        </div>
      </div>

      {/* Modal criar nova tag */}
      {newTagModalOpen() && (
        <CreateTagModal
          initialName={newTagInitialName()}
          onClose={() => setNewTagModalOpen(false)}
          onCreated={(tagName) => addTag(tagName)}
        />
      )}
    </>
  );
};

export default UploadModal;
