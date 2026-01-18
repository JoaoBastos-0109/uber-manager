import { useState } from 'react';

export interface SideBarItemProps {
    title: string;
    icon: string;
    iconSelected: string;
}

function SideBarItem({ title, icon, iconSelected }: SideBarItemProps){

    let [selectedImg, setSelectedImg] = useState(icon);

    const onSelectItem = () => {
        if ( selectedImg === icon)
            setSelectedImg(iconSelected);        
        else
            setSelectedImg(icon);
    }

    return <div className="flex justify-start items-center w-32 px-2 py-1 gap-2
    text-sm text-center border border-transparent rounded
    hover:border-slate-200 cursor-pointer duration-200" onClick={onSelectItem} >
        <img src={selectedImg} className='h-5.5 aspect-square'/>
        <span>{title}</span>
    </div>
}

export default SideBarItem;