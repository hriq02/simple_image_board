import { Component, createSignal, JSX } from "solid-js";

interface CreateTagModalProps {
  initialName?: string;
  onClose: () => void;
  onCreated: (tagName: string) => void;
}

const CreateTagModal: Component<CreateTagModalProps> = (props) => {
  const [name, setName] = createSignal(props.initialName || "");
  const [tagType, setTagType] = createSignal("");

  const handleCreate = async () => {
    if (!name() || !tagType()) return;

    try {
      const res = await fetch("http://localhost:8000/new_tag", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ name: name(), tag_type: tagType() }),
      });

      if (!res.ok) throw new Error("Falha ao criar tag");

      props.onCreated(name());
      props.onClose();
      setName("");
      setTagType("");
    } catch (err) {
      console.error(err);
      alert("Erro ao criar tag");
    }
  };

  return (
    <div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-[9999]">
      <div class="bg-white rounded-2xl shadow-lg w-80 p-6 relative z-[10000]">
        <h2 class="text-lg font-bold mb-4">Criar nova tag</h2>
        <input
          type="text"
          placeholder="Nome da tag"
          class="border rounded p-2 w-full mb-2"
          value={name()}
          onInput={(e) => setName(e.currentTarget.value)}
        />
        <input
          type="text"
          placeholder="Tipo da tag"
          class="border rounded p-2 w-full mb-4"
          value={tagType()}
          onInput={(e) => setTagType(e.currentTarget.value)}
        />
        <div class="flex justify-end gap-2">
          <button
            class="px-4 py-2 rounded bg-gray-300 hover:bg-gray-400"
            onClick={props.onClose}
          >
            Cancelar
          </button>
          <button
            class="px-4 py-2 rounded bg-blue-600 text-white hover:bg-blue-700"
            onClick={handleCreate}
          >
            Criar
          </button>
        </div>
      </div>
    </div>
  );
};

export default CreateTagModal;
