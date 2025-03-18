import { uuidv4 } from '../utils/uuid';
import { Store } from 'tauri-plugin-svelte';

export type ITab = {
    id: string;
    name: string;
    content: string;
    result: string;
    active?: boolean;
    errors: string;
}

const defaultValue: ITab[] = [
    {
        id: uuidv4(),
        name: `New Tab`,
        content: "",
        result: "",
        active: true,
        errors: ""
    }
];

export const storageStore = new Store('storage2', { tabs: defaultValue }, {
    saveOnChange: true,
    saveStrategy: 'debounce',
    saveInterval: 500
});

storageStore.set({tabs: defaultValue});

export const addTab = () => {
    storageStore.update(({ tabs }) => {
        const newTab: ITab = {
            id: uuidv4(),
            name: `New Tab`,
            content: "",
            result: "",
            active: true,
            errors: ""
        };
        const updatedTabs = [
            ...tabs.map((tab) => ({ ...tab, active: false })), // Deactivate all
            newTab,
        ];

        return { tabs: updatedTabs };
    })
}

export const updateTab = (tabId: string, updatedData: Partial<ITab>) => {
    storageStore.update(({ tabs }) => {
        const updatedTabs = tabs.map(tab => tab.id === tabId ? { ...tab, ...updatedData } : tab);

        return { tabs: updatedTabs };
    })
}

export const activateTab = (tabId: string) => {
    storageStore.update(({ tabs }) => {
        const updatedTabs = tabs.map(tab => tab.id === tabId ? { ...tab, active: true } : { ...tab, active: false });

        return { tabs: updatedTabs }
    })
}

export const removeTab = (tabId: string, tabs: ITab[]) => {
    
    let currentTabs = tabs;
    const tab = currentTabs.find(tab => tab.id === tabId);


    storageStore.update(({ tabs }) => ( { tabs: tabs.filter(tab => tab.id !== tabId) } ));


    if (tab.active) {

        const removedTabIndex = currentTabs.findIndex((tab) => tab.id === tabId);

        if (removedTabIndex === -1) return;

        const nextTab = 
            removedTabIndex < currentTabs.length - 1
                ? currentTabs[removedTabIndex + 1]
                : currentTabs[removedTabIndex - 1];

        
    
        nextTab.active = true;
        updateTab(nextTab.id, nextTab);
    }
}

storageStore.start().then();
