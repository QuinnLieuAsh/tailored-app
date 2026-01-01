"use client"
import { listen } from '@tauri-apps/api/event';
import React, { useState, useEffect } from 'react';

export default function Home() {

const [term, setTerm] = useState('');

type DefinitionEvent = {
    term: string;
    definition: string;
};



useEffect(() => {
    let unlistenFn: () => void;

    const setupListener = async () => {
        unlistenFn = await listen<DefinitionEvent>('defintion_event', (event) => {
        setTerm(event.payload.term);
        });
    };

    setupListener();

    return () => {
      if (unlistenFn) {//TODO: Im still confused how this unsubscribes 
        unlistenFn(); // Unsubscribe from the event when the component unmounts
      }
    };

},[]);

  return (
    <div>
        {term}
    </div>
  );
}
